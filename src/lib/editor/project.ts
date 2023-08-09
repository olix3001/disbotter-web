import type { Writable } from 'svelte/store';
import {
	NodeConnectionType,
	type ENode,
	type NodeFlow,
	type NodeIO,
	type NodeConnection,
	type NodeType
} from './node';
import { v4 as uuidv4 } from 'uuid';
import fs from 'fs';

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
				// Safety checks
				// 1: Check if connection does not connect to itself
				if (conn.from === conn.to) return;

				// 2: Check if this input is not already connected
				flow.connections = flow.connections.filter(
					(c) => !(c.to === conn.to && c.toKey === conn.toKey)
				);

				// 3: Check if flow output does not connect to anything else
				if (conn.type === NodeConnectionType.Flow) {
					flow.connections = flow.connections.filter(
						(c) => !(c.from === conn.from && c.fromKey === conn.fromKey)
					);
				}

				// Add the connection
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

	public isEditing(target: Command): boolean {
		return this.currentlyEditing?.type === 'command' && this.currentlyEditing.command === target;
	}

	public setEditing(target: Command): void {
		if (target instanceof Command) {
			this.currentlyEditing = { type: 'command', command: target };
		}
	}
}

export async function loadNodeDeclarations(file: string): Promise<NodeType[]> {
	const nodes = await fetch(file).then((res) => res.json());
	return nodes;
}

let commandAvailableNodes: NodeType[] = [];

if (typeof document !== 'undefined') {
	loadNodeDeclarations('/generated/node_declarations.json').then((nodes) => {
		commandAvailableNodes = nodes;
		console.log(commandAvailableNodes);
	});
}

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
