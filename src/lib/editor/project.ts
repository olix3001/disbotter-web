import type { Writable } from 'svelte/store';
import type { NodeFlow, NodeIO } from './node';
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
}

const commandAvailableNodes = [
	{
		id: 'onCommand',
		title: 'On Command',
		description: 'Triggered when a command is executed',
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
