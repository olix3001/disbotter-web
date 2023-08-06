export const projectKey = Symbol('disbotter project');

export class DisbotterProject {
	public name: string;

	constructor(name: string) {
		this.name = name;
	}
}
