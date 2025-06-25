export function allDefined<T extends object>(
	obj: T,
	skipKeys: (keyof T)[] = [],
): boolean {
	const keys = Object.keys(obj).filter(
		(k) => !skipKeys.includes(k as keyof T),
	);
	return (keys as Array<keyof T>).every((key) => obj[key] !== undefined);
}
