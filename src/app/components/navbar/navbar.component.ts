import { Component } from '@angular/core';
import { Router, RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';

@Component({
  selector: 'app-navbar',
  standalone: true,
  imports: [RouterModule],
  templateUrl: './navbar.component.html',
  styleUrl: './navbar.component.scss'
})
export class NavbarComponent {
  async exit(): Promise<void>{
    try{
        await invoke("tauri_kill");
      }
    catch(error) {
      alert("Error")
    }
  }

}
