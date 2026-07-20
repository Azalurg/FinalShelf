import { ComponentFixture, TestBed } from "@angular/core/testing";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { SettingsPageComponent } from "./settings.component";
import { NotificationService } from "../../shared/services/notification.service";
import { THEME_STORAGE_KEY } from "../../shared/constants/theme.constants";

describe("SettingsPageComponent", () => {
  let component: SettingsPageComponent;
  let fixture: ComponentFixture<SettingsPageComponent>;
  let ipcSpy: jasmine.Spy;

  const notificationMock = {
    success: jasmine.createSpy("success"),
    error: jasmine.createSpy("error"),
    info: jasmine.createSpy("info"),
    warning: jasmine.createSpy("warning"),
  } satisfies Partial<NotificationService>;

  beforeEach(async () => {
    localStorage.clear();
    document.body.className = "";

    // Route Tauri IPC through a spy. `listen("scan-progress", ...)` in ngOnInit
    // resolves through the "plugin:event|listen" command, so it needs a reply.
    ipcSpy = jasmine.createSpy("ipc").and.callFake((cmd: string) => {
      if (cmd === "get_all_absolute_path_command") {
        return Promise.resolve([]);
      }
      if (cmd === "get_version_command") {
        return Promise.resolve("0.0.0");
      }
      if (cmd === "plugin:event|listen") {
        return Promise.resolve(1);
      }
      return Promise.resolve(null);
    });
    mockIPC((cmd, args) => ipcSpy(cmd, args));

    await TestBed.configureTestingModule({
      imports: [SettingsPageComponent],
      providers: [{ provide: NotificationService, useValue: notificationMock }],
    }).compileComponents();

    fixture = TestBed.createComponent(SettingsPageComponent);
    component = fixture.componentInstance;
  });

  afterEach(() => {
    clearMocks();
    document.body.className = "";
    localStorage.clear();
  });

  it("applies saved theme on init", async () => {
    localStorage.setItem(THEME_STORAGE_KEY, "default");

    fixture.detectChanges();
    await fixture.whenStable();

    expect(component.selectedTheme).toBe("default");
    expect(document.body.classList.contains("default")).toBeTrue();
    expect(ipcSpy).toHaveBeenCalled();
  });

  it("updates theme selection and persists to storage", () => {
    const event = { target: { value: "default" } } as unknown as Event;

    component.updateTheme(event);

    expect(localStorage.getItem(THEME_STORAGE_KEY)).toBe("default");
    expect(document.body.classList.contains("default")).toBeTrue();
  });
});
