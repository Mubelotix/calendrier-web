export {};
let wasmMemory = null;

export function setMemory(memory) {
	wasmMemory = memory;
}

export function strcpy(destination, source) {
	return destination;
}

export function toupper(character) {
	if (character >= 97 && character <= 122) {
		return character - 32;
	}

	return character;
}

export function calloc() {
	return 0;
}

export function malloc() {
	return 0;
}

export function fclose() {
	return 0;
}

export function fopen() {
	return 0;
}

export function fread() {
	return 0;
}

export function free() {}

export function fseek() {
	return 0;
}