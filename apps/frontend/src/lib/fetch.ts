export async function fetch_json<T>(url: string): Promise<T> {
	const response = await fetch(url);
	return response.json();
}