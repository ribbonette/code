export async function fetch_json<T>(url: string, init?: RequestInit): Promise<T> {
	const response = await fetch(url, init);
	if (response.status < 200 || response.status >= 300)
		throw new Error(`Failed to fetch: ${response.status} ${response.statusText}`);
	
	return response.json();
}