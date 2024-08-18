import { convertFileSrc } from "@tauri-apps/api/tauri";

export function convertImgPathBook(path: string): string {
    if (!path){
      return 'assets/book.jpg';
    }
    return convertFileSrc(path);
    
  }

  export function convertImgPathAuthor(path: string): string {
    if (!path){
      return 'assets/author.jpg';
    }
    return convertFileSrc(path);
    
  }