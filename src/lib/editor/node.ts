export interface NodeType {
	id: string;
	title: string;
	description: string;
	category: string;
	color: string;
	icon: string;

	inputs: NodeIOType;
	outputs: NodeIOType;

	defaultHardcoded?: NodeIO;

	action: (node: Node, inputs: NodeIO) => NodeIO;
}

export type NodeIOType = {
	[key: string]: { type: { type: NodeConnectionType; structType?: string }; name: string };
};
export type NodeIO = { [key: string]: any };

export enum NodeConnectionType {
	Flow,
	Number,
	Text,
	Boolean,
	Structure,
	Any
}

export function getNodeConnectionTypeColor(type: NodeConnectionType) {
	switch (type) {
		case NodeConnectionType.Flow:
			return '#ffffff'; // White
		case NodeConnectionType.Number:
			return '#2f904b'; // Green
		case NodeConnectionType.Text:
			return '#eeab2c'; // Orange
		case NodeConnectionType.Boolean:
			return '#ee5339'; // Red
		case NodeConnectionType.Structure:
			return '#427ade'; // Blue
		case NodeConnectionType.Any:
			return '#8338f9'; // Purple
	}
}

export interface ENode {
	uid: string;

	type: NodeType;
	x: number;
	y: number;

	iPorts: { [key: string]: SVGSVGElement };
	oPorts: { [key: string]: SVGSVGElement };

	inputHardcoded: { [key: string]: any };

	self?: HTMLElement;
}

export interface NodeConnection {
	type: NodeConnectionType;
	sType?: string;

	from: ENode | null;
	fromKey: string | null;
	to: ENode | null;
	toKey: string | null;
}

export interface NodeFlow {
	nodes: ENode[];
	connections: NodeConnection[];

	availableNodes: NodeType[];
}
