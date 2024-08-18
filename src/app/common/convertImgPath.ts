import { convertFileSrc } from "@tauri-apps/api/tauri";

export function convertImgPath(path: string): string {
    if (!path){
      return 'assets/logo.svg';
    }
    return convertFileSrc(path);
    
  }