import { Dokki } from "./dokki";

export type Runtime = {
	dokki: Dokki;
};

export const newRuntime = (dokki: Dokki) => {
	return {
		dokki,
	} as Runtime;
};
