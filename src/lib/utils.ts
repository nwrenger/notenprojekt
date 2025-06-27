type AllDefinedExcept<T extends Record<string, any>, K extends keyof T> = {
	[P in Exclude<keyof T, K>]-?: NonNullable<T[P]>;
} & Pick<T, K>;

export function allDefined<T extends Record<string, any>, K extends keyof T>(
	obj: T,
	skipKeys: K[],
): obj is T & AllDefinedExcept<T, K> {
	const allKeys = Object.keys(obj) as Array<keyof T>;
	const keysToCheck = allKeys.filter(
		(key): key is Exclude<keyof T, K> => !skipKeys.includes(key as K),
	);
	return keysToCheck.every(
		(key) => obj[key] !== undefined && obj[key] !== null && obj[key],
	);
}
