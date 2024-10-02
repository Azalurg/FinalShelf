import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GenresDetailsComponent } from './genres-details.component';

describe('GenresDetailsComponent', () => {
  let component: GenresDetailsComponent;
  let fixture: ComponentFixture<GenresDetailsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [GenresDetailsComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(GenresDetailsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
