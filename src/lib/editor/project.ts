import type { Writable } from 'svelte/store';
import {
	NodeConnectionType,
	type ENode,
	type NodeFlow,
	type NodeIO,
	type NodeConnection
} from './node';
import { v4 as uuidv4 } from 'uuid';

export const projectKey = Symbol('disbotter project');

export type ProjectContext = Writable<DisbotterProject>;
export class DisbotterProject {
	public name: string;

	public commands: Command[] = [];
	public currentlyEditing: { type: 'command'; command?: Command } | null = null;

	public currentConnection: NodeConnection | null = null;

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

	public createConnection(conn: NodeConnection): void {
		const flow = this.getCurrentFlow();

		if (flow) {
			// Check if the connection already exists
			const existingConnection = flow.connections.find((c) => {
				return (
					c.from === conn.from &&
					c.fromKey === conn.fromKey &&
					c.to === conn.to &&
					c.toKey === conn.toKey
				);
			});

			if (existingConnection) {
				// Break the connection
				flow.connections = flow.connections.filter((c) => c !== existingConnection);
			} else {
				// Create the connection
				flow.connections.push(conn);
			}
		}
	}

	public addNode(node: ENode): void {
		if (this.currentlyEditing?.type === 'command') {
			this.currentlyEditing.command?.flow.nodes.push(node);
		}
	}

	public removeNodes(nodes: ENode[]): void {
		if (this.currentlyEditing?.type === 'command') {
			if (this.currentlyEditing.command) {
				for (const node of nodes) {
					this.removeRelatedConnections(node);
				}
				this.currentlyEditing.command.flow.nodes = this.currentlyEditing.command.flow.nodes.filter(
					(node) => !nodes.includes(node)
				);
			}
		}
	}

	public removeRelatedConnections(node: ENode): void {
		if (this.currentlyEditing?.type === 'command') {
			if (this.currentlyEditing.command) {
				this.currentlyEditing.command.flow.connections =
					this.currentlyEditing.command.flow.connections.filter(
						(conn) => conn.from !== node && conn.to !== node
					);
			}
		}
	}
}

const commandAvailableNodes: any = [
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
				type: NodeConnectionType.Flow,
				name: 'Flow'
			},
			interaction: {
				type: NodeConnectionType.Structure,
				name: 'Interaction',
				struct: {}
			}
		},

		action: (node: Node, inputs: NodeIO): NodeIO => {
			return {};
		}
	},
	{
		id: 'Reply',
		title: 'Reply',
		description: 'Replies to the interaction/message',
		category: 'Actions',
		color: '#e91e63',
		icon: '/icons/editor/trigger.png',

		inputs: {
			__flow_in__: {
				type: NodeConnectionType.Flow,
				name: 'Flow'
			},
			target: {
				type: NodeConnectionType.Structure,
				name: 'Target',
				struct: {}
			}
		},
		outputs: {
			__flow_out__: {
				type: NodeConnectionType.Flow,
				name: 'Flow'
			}
		},

		action: (node: Node, inputs: NodeIO): NodeIO => {
			return {};
		}
	},
	{
		id: 'Test',
		title: 'IO Test',
		description: 'Contains all the IO types',
		category: 'Development',
		color: '#e91e63',
		icon: '/icons/editor/trigger.png',

		inputs: {
			__flow_in__: {
				type: NodeConnectionType.Flow,
				name: 'Flow'
			},
			number: {
				type: NodeConnectionType.Number,
				name: 'Number'
			},
			text: {
				type: NodeConnectionType.Text,
				name: 'Text'
			},
			boolean: {
				type: NodeConnectionType.Boolean,
				name: 'Boolean'
			},
			structure: {
				type: NodeConnectionType.Structure,
				name: 'Structure',
				struct: {
					foo: {
						type: NodeConnectionType.Text,
						name: 'Foo'
					},
					bar: {
						type: NodeConnectionType.Number,
						name: 'Bar'
					}
				}
			},
			any: {
				type: NodeConnectionType.Any,
				name: 'Any'
			}
		},
		outputs: {
			__flow_out__: {
				type: NodeConnectionType.Flow,
				name: 'Flow'
			},
			number: {
				type: NodeConnectionType.Number,
				name: 'Number'
			},
			text: {
				type: NodeConnectionType.Text,
				name: 'Text'
			},
			boolean: {
				type: NodeConnectionType.Boolean,
				name: 'Boolean'
			},
			structure: {
				type: NodeConnectionType.Structure,
				name: 'Structure',
				struct: {
					foo: {
						type: NodeConnectionType.Text,
						name: 'Foo'
					},
					bar: {
						type: NodeConnectionType.Number,
						name: 'Bar'
					}
				}
			},
			any: {
				type: NodeConnectionType.Any,
				name: 'Any'
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
