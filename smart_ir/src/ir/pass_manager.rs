// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use downcast_rs::Downcast;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
pub trait PassConcept: 'static {
    type IRUnit;

    fn run(
        &self,
        ir: &mut Self::IRUnit,
        am: &mut AnalysisManager<Self::IRUnit>,
    ) -> PreservedAnalysis;
}

pub struct PassManager<IRUnit> {
    passes: Vec<Box<dyn PassConcept<IRUnit = IRUnit>>>,
}

impl<IRUnit: 'static> Default for PassManager<IRUnit> {
    fn default() -> Self {
        Self { passes: vec![] }
    }
}

impl<IRUnit: 'static> PassManager<IRUnit> {
    pub fn add_pass<PassImpl>(&mut self, pass: PassImpl)
    where
        PassImpl: PassConcept<IRUnit = IRUnit> + 'static,
    {
        self.passes.push(Box::new(pass));
    }
}

impl<IRUnit: 'static> PassConcept for PassManager<IRUnit> {
    type IRUnit = IRUnit;
    fn run(&self, ir: &mut IRUnit, am: &mut AnalysisManager<IRUnit>) -> PreservedAnalysis {
        let mut pa = PreservedAnalysis::all();

        for pass in self.passes.iter() {
            let pass_pa = pass.run(ir, am);
            am.invalidate(ir, &pass_pa);
            pa.intersect(&pass_pa);
        }
        pa
    }
}

pub trait AnalysisResultConcept: Downcast {
    type IRUnit;
    fn invalidate(&self, ir: &Self::IRUnit, am: &PreservedAnalysis) -> bool;
}

downcast_rs::impl_downcast!(AnalysisResultConcept assoc IRUnit);

pub trait AnalysisPassModel: AnalysisPassConcept {
    const NAME: &'static str;
    type AnalysisResult: AnalysisResultConcept<IRUnit = Self::IRUnit>;
}

pub type AnalysisPassKey = usize;

pub trait AnalysisPassConcept: 'static {
    type IRUnit;

    fn run(&self, ir_unit: &Self::IRUnit) -> Rc<dyn AnalysisResultConcept<IRUnit = Self::IRUnit>>;
    fn id() -> AnalysisPassKey
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct PreservedAnalysis {
    preserved_ids: HashSet<AnalysisPassKey>,
}

impl PreservedAnalysis {
    pub fn none() -> Self {
        Self {
            preserved_ids: HashSet::default(),
        }
    }

    pub fn all() -> Self {
        Self {
            preserved_ids: HashSet::from([Self::all_analysis_id()]),
        }
    }

    pub fn are_all_preserved(&self) -> bool {
        self.preserved_ids.contains(&Self::all_analysis_id())
    }

    pub fn preserved<AnalysisImpl>(&self) -> bool
    where
        AnalysisImpl: AnalysisPassConcept,
    {
        self.are_all_preserved() || self.preserved_ids.contains(&AnalysisImpl::id())
    }

    fn all_analysis_id() -> AnalysisPassKey {
        static ID: &str = "all_analysis";
        std::ptr::addr_of!(ID) as AnalysisPassKey
    }

    pub fn preserve<AnalysisImpl>(&mut self)
    where
        AnalysisImpl: AnalysisPassConcept,
    {
        if self.are_all_preserved() {
            return;
        }
        self.preserved_ids.insert(AnalysisImpl::id());
    }

    pub fn intersect(&mut self, other: &Self) {
        if other.are_all_preserved() {
            return;
        }
        if self.are_all_preserved() {
            *self = other.clone();
            return;
        }
        self.preserved_ids = self
            .preserved_ids
            .intersection(&other.preserved_ids)
            .cloned()
            .collect();
    }
}
pub type IRUnitKey = usize;
pub struct AnalysisManager<IRUnit> {
    analysis_passes: HashMap<AnalysisPassKey, Box<dyn AnalysisPassConcept<IRUnit = IRUnit>>>,
    analysis_result_maps:
        HashMap<IRUnitKey, HashMap<IRUnitKey, Rc<dyn AnalysisResultConcept<IRUnit = IRUnit>>>>,
}

impl<IRUnit: 'static> Default for AnalysisManager<IRUnit> {
    fn default() -> Self {
        Self {
            analysis_passes: HashMap::default(),
            analysis_result_maps: HashMap::default(),
        }
    }
}

impl<IRUnit: 'static> AnalysisManager<IRUnit> {
    pub fn lookup_analysis<AnalysisImpl>(&self) -> Option<&dyn AnalysisPassConcept<IRUnit = IRUnit>>
    where
        AnalysisImpl: AnalysisPassModel<IRUnit = IRUnit>,
    {
        self.analysis_passes
            .get(&AnalysisImpl::id())
            .map(|x| x.as_ref())
    }

    pub fn register_analysis<AnalysisImpl, AnalysisBuilder>(&mut self, builder: AnalysisBuilder)
    where
        AnalysisBuilder: FnOnce() -> AnalysisImpl,
        AnalysisImpl: AnalysisPassConcept<IRUnit = IRUnit>,
    {
        self.analysis_passes
            .insert(AnalysisImpl::id(), Box::new(builder()));
    }

    pub fn get_result<AnalysisImpl>(&mut self, ir: &IRUnit) -> Rc<AnalysisImpl::AnalysisResult>
    where
        AnalysisImpl: AnalysisPassModel<IRUnit = IRUnit>,
    {
        if let Some(res) = self.get_cached_result::<AnalysisImpl>(ir) {
            return res;
        }
        let unit_id = ir as *const IRUnit as IRUnitKey;
        let analysis_id = AnalysisImpl::id();
        self.analysis_result_maps
            .entry(unit_id)
            .or_insert_with(HashMap::default);

        let result = {
            let pass = self
                .lookup_analysis::<AnalysisImpl>()
                .unwrap_or_else(|| panic!("can't find analysis pass {}", AnalysisImpl::NAME));
            pass.run(ir)
        };
        self.analysis_result_maps
            .get_mut(&unit_id)
            .unwrap()
            .insert(analysis_id, result.clone());

        match result.downcast_rc::<AnalysisImpl::AnalysisResult>() {
            Ok(result) => result,
            Err(_) => panic!(
                "cast analysis reulst to {} failed",
                std::any::type_name::<AnalysisImpl::AnalysisResult>(),
            ),
        }
    }

    pub fn get_cached_result<AnalysisImpl>(
        &mut self,
        ir: &IRUnit,
    ) -> Option<Rc<AnalysisImpl::AnalysisResult>>
    where
        AnalysisImpl: AnalysisPassModel<IRUnit = IRUnit>,
    {
        let unit_id = ir as *const IRUnit as IRUnitKey;
        match self.analysis_result_maps.get(&unit_id) {
            Some(result_map) => result_map.get(&AnalysisImpl::id()).map(|result| {
                result
                    .clone()
                    .downcast_rc::<AnalysisImpl::AnalysisResult>()
                    .unwrap_or_else(|_| {
                        panic!(
                            "cast analysis resulst to {} failed",
                            std::any::type_name::<AnalysisImpl::AnalysisResult>()
                        )
                    })
            }),
            None => None,
        }
    }

    pub fn invalidate(&mut self, ir: &IRUnit, pa: &PreservedAnalysis) {
        if pa.are_all_preserved() {
            return;
        }
        let unit_id = ir as *const IRUnit as IRUnitKey;
        if !self.analysis_result_maps.contains_key(&unit_id) {
            return;
        }
        let mut is_result_invalidated = HashMap::<AnalysisPassKey, bool>::default();
        let result_map = self.analysis_result_maps.get_mut(&unit_id).unwrap();

        for (pass_id, result) in result_map.iter() {
            is_result_invalidated.insert(*pass_id, result.invalidate(ir, pa));
        }

        if !is_result_invalidated.is_empty() {
            for (id, invalidated) in is_result_invalidated.iter() {
                if *invalidated {
                    result_map.remove(id);
                }
            }
        }
        if result_map.is_empty() {
            self.analysis_result_maps.remove(&unit_id);
        }
    }

    pub fn clear(&mut self) {
        self.analysis_result_maps.clear();
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, rc::Rc};

    use crate::ir::pass_manager::AnalysisPassKey;

    use super::{
        AnalysisManager, AnalysisPassConcept, AnalysisPassModel, AnalysisResultConcept,
        PassConcept, PassManager, PreservedAnalysis,
    };

    pub struct TestUnit {
        pub data: Vec<u32>,
    }

    struct FooPass {}

    impl PassConcept for FooPass {
        type IRUnit = TestUnit;

        fn run(
            &self,
            ir: &mut Self::IRUnit,
            am: &mut AnalysisManager<Self::IRUnit>,
        ) -> PreservedAnalysis {
            println!("run in foo_pass");
            let foo_result = am.get_result::<FooAnalysis>(ir);
            let bar_result = am.get_result::<BarAnalysis>(ir);
            if !ir.data.is_empty() {
                ir.data[0] = foo_result.0 + bar_result.0;
            }
            let mut pa = PreservedAnalysis::none();
            pa.preserve::<BarAnalysis>();
            pa
        }
    }
    struct FooAnalysis {}

    struct FooResult(u32);

    impl AnalysisPassConcept for FooAnalysis {
        type IRUnit = TestUnit;

        fn run(&self, ir: &Self::IRUnit) -> Rc<dyn AnalysisResultConcept<IRUnit = Self::IRUnit>> {
            println!("run in {}", FooAnalysis::NAME);
            Rc::new(FooResult(ir.data.iter().fold(0, |acc, x| acc + x)))
        }

        fn id() -> AnalysisPassKey
        where
            Self: Sized,
        {
            static ID: &str = FooAnalysis::NAME;
            std::ptr::addr_of!(ID) as AnalysisPassKey
        }
    }

    impl AnalysisPassModel for FooAnalysis {
        type AnalysisResult = FooResult;

        const NAME: &'static str = "foo_analysis";
    }

    impl AnalysisResultConcept for FooResult {
        type IRUnit = TestUnit;

        fn invalidate(&self, _ir: &Self::IRUnit, pa: &PreservedAnalysis) -> bool {
            !pa.preserved::<FooAnalysis>()
        }
    }

    struct BarAnalysis {}

    struct BarResult(u32);

    impl AnalysisPassConcept for BarAnalysis {
        type IRUnit = TestUnit;

        fn run(&self, ir: &Self::IRUnit) -> Rc<dyn AnalysisResultConcept<IRUnit = Self::IRUnit>> {
            println!("run in {}", BarAnalysis::NAME);
            Rc::new(BarResult(ir.data.len() as u32))
        }

        fn id() -> AnalysisPassKey
        where
            Self: Sized,
        {
            static ID: &str = BarAnalysis::NAME;
            std::ptr::addr_of!(ID) as AnalysisPassKey
        }
    }

    impl AnalysisPassModel for BarAnalysis {
        type AnalysisResult = BarResult;

        const NAME: &'static str = "bar_analysis";
    }

    impl AnalysisResultConcept for BarResult {
        type IRUnit = TestUnit;

        fn invalidate(&self, _ir: &Self::IRUnit, pa: &PreservedAnalysis) -> bool {
            !pa.preserved::<BarAnalysis>()
        }
    }

    #[test]
    fn test() {
        let mut pass_manager = PassManager {
            passes: Vec::default(),
        };
        let mut analysis_manager = AnalysisManager {
            analysis_passes: HashMap::default(),
            analysis_result_maps: HashMap::default(),
        };

        let mut ir = TestUnit {
            data: vec![1, 2, 3, 4, 5, 6, 7],
        };

        analysis_manager.register_analysis(|| FooAnalysis {});
        analysis_manager.register_analysis(|| BarAnalysis {});

        pass_manager.add_pass(FooPass {});
        pass_manager.add_pass(FooPass {});
        pass_manager.run(&mut ir, &mut analysis_manager);

        // data[0] = (28 + 7) + 27 +7 = 69
        assert_eq!(ir.data, vec![69, 2, 3, 4, 5, 6, 7])
    }
}
