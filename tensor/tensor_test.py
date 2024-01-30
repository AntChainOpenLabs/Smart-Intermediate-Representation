import json
import torch

from torch_geometric.data import HeteroData


def read_file():
    with open("tensor/data.json", "r") as file:
        json_data = file.read()

    # 反序列化 JSON
    data = json.loads(json_data)
    return data


def cfg2data(cfg: dict) -> HeteroData:
    data = HeteroData()
    cfg_edges = cfg["cfg_edges"]
    cfg_feats = cfg["cfg_feats"]

    node_obj2index = dict()
    node_type2feats = dict()
    edge_type2edges = dict()
    edge_type2attrs = dict()

    node_type2feats["BasicBlock"] = list()

    q = []

    edge_type2edges["BasicBlock", "BasicBlock"] = list()
    edge_type2attrs["BasicBlock", "BasicBlock"] = list()
    node_type2feats["BasicBlock"] = list()

    for bb in cfg_feats:
        bb_id = bb["idx_in_parent"]
        q.append(([bb_id], bb))
        node_obj2index[id(bb)] = bb_id
        node_type2feats["BasicBlock"].append([0])
    


    for bb in cfg_feats:
        bb_id = bb["idx_in_parent"]
        node_obj2index[id(bb)] = bb_id
        if cfg_edges.get(str(bb_id)):
            targets = cfg_edges[str(bb_id)]
            for (idx, target) in enumerate(targets):
                edge_type2edges["BasicBlock", "BasicBlock"].append(
                    [bb_id, target]
                )
                edge_type2attrs["BasicBlock", "BasicBlock"].append(
                    [idx]
                )


                

    while len(q) > 0:
        path, node = q.pop(0)
        for sub_node in node["children"]:
            sub_path = path.copy()
            sub_path.append(sub_node["idx_in_parent"])
            q.append((sub_path, sub_node))

            # add edges to sub node
            if not node_type2feats.get(sub_node["inner_kind"]):
                node_type2feats[sub_node["inner_kind"]] = list()

            idx = len(node_type2feats[sub_node["inner_kind"]])
            node_obj2index[id(sub_node)] = idx
            edge_type = node["inner_kind"], sub_node["inner_kind"]

            if not edge_type2edges.get(edge_type):
                edge_type2edges[edge_type] = list()
                edge_type2attrs[edge_type] = list()
                
            edge_type2edges[edge_type].append(
                [node_obj2index[id(node)], node_obj2index[id(sub_node)]]
            )
            edge_type2attrs[edge_type].append([sub_node["idx_in_parent"]])

            feats = [
                sub_node["depth"],
                # *path,
            ]
            # add node feats
            node_type2feats[sub_node["inner_kind"]].append(feats)

    # print(node_type2feats)
    print("node feats")
    for node, feats in node_type2feats.items():
        print(node)
        print(feats)

    print()
    
    print("edge")
    for edge_type, edge_index in edge_type2edges.items():
        print(edge_type)
        print(edge_index)
        # data[edge_type].edge_index = torch.tensor(edge_index).t().contiguous()
    print()
    print("edge feat")
    for edge_type, edge_attr in edge_type2attrs.items():
        # data[edge_type].edge_attr = torch.tensor(edge_attr)
        print(edge_type)
        print(edge_attr)

    for node_type, feats in node_type2feats.items():
        data[node_type].x = torch.tensor(feats)
    for edge_type, edge_index in edge_type2edges.items():
        data[edge_type].edge_index = torch.tensor(edge_index).t().contiguous()
    for edge_type, edge_attr in edge_type2attrs.items():
        data[edge_type].edge_attr = torch.tensor(edge_attr)
    return data


if __name__ == "__main__":
    data = read_file()
    for d in data:
        graph = cfg2data(d)
        # torch.save(graph, "tensor/hetero.pt")
