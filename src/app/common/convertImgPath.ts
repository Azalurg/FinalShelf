import { convertFileSrc } from "@tauri-apps/api/core";

function resolveAbsolutePath(path: string, absolute_path: string): string {
  if (path.startsWith("/") || path.match(/^[a-zA-Z]:\\/)) {
    return path;
  }
  return `${absolute_path.replace(/\/$/, '')}/${path.replace(/^\/+/, '')}`;
}

export function convertImgPathBook(path: string, absolute_path: string): string {
  if (!path) {
    return 'assets/book.jpg';
  }

  const fullPath = resolveAbsolutePath(path, absolute_path);
  return convertFileSrc(fullPath);
}

export function convertImgPathAuthor(path: string, absolute_path: string): string {
  if (!path) {
    return 'assets/author.jpg';
  }

  const fullPath = resolveAbsolutePath(path, absolute_path);
  return convertFileSrc(fullPath);
}
