/**
 * Dokki interface for dokki library.
 */

// @ts-ignore
import dokki from '../Cargo.toml';

export type Context = never;

export interface Dokki {
	new_context(): Context;
	compile(ctx: Context, src: string): Uint8Array;
};

export const loadDokki = async () => {
	const m = await dokki();
	return {
		new_context: m.new_context,
		compile: m.compile,
	} as Dokki;
};
