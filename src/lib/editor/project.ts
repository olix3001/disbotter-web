import type { Writable } from 'svelte/store';
import type { ENode, NodeFlow, NodeIO } from './node';
import { v4 as uuidv4 } from 'uuid';

export const projectKey = Symbol('disbotter project');

export type ProjectContext = Writable<DisbotterProject>;
export class DisbotterProject {
	public name: string;

	public commands: Command[] = [];
	public currentlyEditing: { type: 'command'; command?: Command } | null = null;

	constructor(name: string) {
		this.name = name;
	}

	public addCommand(command: Command): void {
		this.commands.push(command);
		this.currentlyEditing = { type: 'command', command };
	}

	public getCurrentFlow(): NodeFlow | null {
		if (this.currentlyEditing?.type === 'command') {
			return this.currentlyEditing.command?.flow ?? null;
		}

		return null;
	}

	public addNode(node: ENode): void {
		if (this.currentlyEditing?.type === 'command') {
			this.currentlyEditing.command?.flow.nodes.push(node);
		}
	}

	public removeNodes(nodes: ENode[]): void {
		if (this.currentlyEditing?.type === 'command') {
			if (this.currentlyEditing.command) {
				this.currentlyEditing.command.flow.nodes = this.currentlyEditing.command.flow.nodes.filter(
					(node) => !nodes.includes(node)
				);
			}
		}
	}
}

const commandAvailableNodes = [
	{
		id: 'onCommand',
		title: 'On Command',
		description: 'Triggered when a command is executed',
		category: 'Events',
		color: '#e91e63',
		icon: '/icons/editor/trigger.png',

		inputs: {},
		outputs: {
			__flow_out__: {
				type: 'flow',
				name: 'Flow'
			}
		},

		action: (node: Node, inputs: NodeIO): NodeIO => {
			return {};
		}
	},
	{
		id: 'testb',
		title: 'Hello world',
		description:
			'lasdasjhdias hiud hasd hahs edahs iudbnsjk dbashjvg duyasb dygiqwbiu bahjbd sahyb dihasb djhbwqjuhqb hdbsa bdkahs bdhdbqwb jhbdvsa bw',
		category: 'Events',
		color: '#e91e63',
		icon: '/icons/editor/trigger.png',

		inputs: {},
		outputs: {
			__flow_out__: {
				type: 'flow',
				name: 'Flow'
			}
		},

		action: (node: Node, inputs: NodeIO): NodeIO => {
			return {};
		}
	},
	{
		id: 'helloworld',
		title: 'Hello world',
		description: 'Returns a hello world message',
		category: 'Events',
		color: '#e91e63',
		icon: '/icons/editor/trigger.png',

		inputs: {},
		outputs: {
			__flow_out__: {
				type: 'flow',
				name: 'Flow'
			}
		},

		action: (node: Node, inputs: NodeIO): NodeIO => {
			return {};
		}
	},
	{
		id: 'bruh',
		title: 'Bruh',
		description: 'I won\t even bother to describe this',
		category: 'Events',
		color: '#e91e63',
		icon: '/icons/editor/trigger.png',

		inputs: {},
		outputs: {
			__flow_out__: {
				type: 'flow',
				name: 'Flow'
			}
		},

		action: (node: Node, inputs: NodeIO): NodeIO => {
			return {};
		}
	},
	{
		id: 'last',
		title: 'Last one',
		description: 'Last node',
		category: 'Events',
		color: '#e91e63',
		icon: '/icons/editor/trigger.png',

		inputs: {},
		outputs: {
			__flow_out__: {
				type: 'flow',
				name: 'Flow'
			}
		},

		action: (node: Node, inputs: NodeIO): NodeIO => {
			return {};
		}
	}
];

export class Command {
	public uid = uuidv4();
	public name: string;
	public description: string;

	public flow: NodeFlow;

	constructor(name: string, description: string) {
		this.name = name;
		this.description = description;

		this.flow = {
			nodes: [],
			connections: [],
			availableNodes: commandAvailableNodes
		};
	}
}
