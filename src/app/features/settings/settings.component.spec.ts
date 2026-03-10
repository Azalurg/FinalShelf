import { ComponentFixture, TestBed } from "@angular/core/testing";
import * as tauriCore from "@tauri-apps/api/core";
import * as tauriEvent from "@tauri-apps/api/event";
import { SettingsPageComponent } from "./settings.component";
import { NotificationService } from "../../shared/services/notification.service";
import { THEME_STORAGE_KEY } from "../../shared/constants/theme.constants";

describe("SettingsPageComponent", () => {
  let component: SettingsPageComponent;
  let fixture: ComponentFixture<SettingsPageComponent>;
  let invokeSpy: jasmine.Spy;

  const notificationMock = {
    success: jasmine.createSpy("success"),
    error: jasmine.createSpy("error"),
    info: jasmine.createSpy("info"),
    warning: jasmine.createSpy("warning"),
  } satisfies Partial<NotificationService>;

  beforeEach(async () => {
    localStorage.clear();
    document.body.className = "";

    invokeSpy = spyOn(tauriCore, "invoke").and.callFake((command: string) => {
      if (command === "get_all_absolute_path_command") {
        return Promise.resolve([]);
      }
      if (command === "get_version_command") {
        return Promise.resolve("0.0.0");
      }
      return Promise.resolve(null);
    });

    spyOn(tauriEvent, "listen").and.returnValue(Promise.resolve(() => {}));

    await TestBed.configureTestingModule({
      imports: [SettingsPageComponent],
      providers: [{ provide: NotificationService, useValue: notificationMock }],
    }).compileComponents();

    fixture = TestBed.createComponent(SettingsPageComponent);
    component = fixture.componentInstance;
  });

  afterEach(() => {
    document.body.className = "";
    localStorage.clear();
  });

  it("applies saved theme on init", async () => {
    localStorage.setItem(THEME_STORAGE_KEY, "default");

    fixture.detectChanges();
    await fixture.whenStable();

    expect(component.selectedTheme).toBe("default");
    expect(document.body.classList.contains("default")).toBeTrue();
    expect(invokeSpy).toHaveBeenCalled();
  });

  it("updates theme selection and persists to storage", () => {
    const event = { target: { value: "default" } } as unknown as Event;

    component.updateTheme(event);

    expect(localStorage.getItem(THEME_STORAGE_KEY)).toBe("default");
    expect(document.body.classList.contains("default")).toBeTrue();
  });
});
