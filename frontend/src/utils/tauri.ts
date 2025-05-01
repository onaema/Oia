export async function fetchHelloMessage() {
  try {
    const response = await fetch('http://127.0.0.1:3030/api/hello');
    if (!response.ok) {
      throw new Error('Failed to fetch message');
    }
    return await response.json();
  } catch (error) {
    console.error('Error fetching message:', error);
    throw error;
  }
}