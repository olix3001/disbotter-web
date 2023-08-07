export interface NodeType {
	id: string;
	title: string;
	description: string;
	category: string;
	color: string;
	icon: string;

	inputs: NodeIOType;
	outputs: NodeIOType;

	action: (node: Node, inputs: NodeIO) => NodeIO;
}

export type NodeIOType = { [key: string]: { type: string; name: string; struct?: StructureType } };
export type NodeIO = { [key: string]: any };

export interface StructureType {
	[key: string]: NodeConnectionType;
}

export enum NodeConnectionType {
	Flow,
	Number,
	Text,
	Boolean,
	Structure,
	Any
}

export interface ENode {
	id: string;
	type: NodeType;
	x: number;
	y: number;

	inputs: { [key: string]: NodeConnection };
	outputs: { [key: string]: NodeConnection };

	inputHardcoded: { [key: string]: any };
	outputHardcoded: { [key: string]: any };

	self?: HTMLElement;
}

export interface NodeConnection {
	type: NodeConnectionType;
	value: any;

	from: ENode;
	to: ENode;

	start?: HTMLElement;
	end?: HTMLElement;

	self?: HTMLElement;
}

export interface NodeFlow {
	nodes: ENode[];
	connections: NodeConnection[];

	availableNodes: NodeType[];
}
