import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LectorsComponent } from './lectors.component';

describe('LectorsComponent', () => {
  let component: LectorsComponent;
  let fixture: ComponentFixture<LectorsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LectorsComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(LectorsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
