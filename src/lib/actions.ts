"use server"

export async function fetchData(url: string) {
  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    const data = await response.json(); // Assuming the response is JSON
    return data;
  } catch (error) {
    console.error("Error fetching data: ", error);
  }
}
