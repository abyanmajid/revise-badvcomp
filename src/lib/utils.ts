import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function parseStringToArray(str: string) {
  const regex = /\((\d+)\)\s*(.+)/;
  const match = str.match(regex);

  if (match) {
    const id = parseInt(match[1], 10); // Convert ID to a number
    const topic = match[2];
    return [topic, id];
  }

  return [];
}
