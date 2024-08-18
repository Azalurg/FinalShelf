import { convertFileSrc } from "@tauri-apps/api/tauri";

export function convertImgPath(path: string): string {
    if (!path){
      return 'assets/book.jpg';
    }
    return convertFileSrc(path);
    
  }