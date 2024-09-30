import { ComponentFixture, TestBed } from '@angular/core/testing';

import { LectorsDetailsComponent } from './lectors-details.component';

describe('LectorsDetailsComponent', () => {
  let component: LectorsDetailsComponent;
  let fixture: ComponentFixture<LectorsDetailsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [LectorsDetailsComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(LectorsDetailsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
