// #![feature(const_fn)]
extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate libc;

use winapi::windef::HWND;
use winapi::windef::HDC;
use winapi::windef::HMENU;
use winapi::windef::HBRUSH;
use winapi::windef::LPRECT;
use winapi::windef::RECT;

use winapi::minwindef::LPVOID;
use winapi::minwindef::HINSTANCE;
use winapi::minwindef::UINT;
use winapi::minwindef::DWORD;
use winapi::minwindef::WPARAM;
use winapi::minwindef::LPARAM;
use winapi::minwindef::LRESULT;
use winapi::minwindef::HRGN;

use winapi::winnt::LPCWSTR;
use winapi::winnt::LPCSTR;

use winapi::winuser::WS_OVERLAPPEDWINDOW;
use winapi::winuser::WS_VISIBLE;
use winapi::winuser::WNDCLASSW;
use winapi::winuser::LPPAINTSTRUCT;
use winapi::winuser::PAINTSTRUCT;
use winapi::winuser::DT_SINGLELINE;
use winapi::winuser::DT_CENTER;
use winapi::winuser::DT_VCENTER;
use winapi::winuser::RDW_INVALIDATE;
use winapi::winuser::RDW_NOERASE;
use winapi::winuser::RDW_INTERNALPAINT;
use winapi::winuser::CS_HREDRAW;
use winapi::winuser::CS_VREDRAW;
use winapi::winuser::WS_CHILD;
use winapi::winuser::BS_DEFPUSHBUTTON;
use winapi::winuser::GWL_HINSTANCE;

use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::mem;
use std::ptr;

use libc::c_int;

enum Handle {
    Button,
    Window,
}

impl Handle {
    fn value(&self) -> HMENU {
        match *self {
            Handle::Button => 1001 as HMENU,
            Handle::Window => 0 as HMENU,
        }
    }
} 

pub unsafe extern "system" fn windowProc(hwnd: HWND,
    msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {

    let hdc: HDC;
    let lpPaintStruct: LPPAINTSTRUCT = libc::malloc(mem::size_of::<PAINTSTRUCT>() as libc::size_t) as *mut PAINTSTRUCT;;
    let lpRect: LPRECT = libc::malloc(mem::size_of::<RECT>() as libc::size_t) as *mut RECT;
    let buttonDontPressMe: HWND;
    let buttonPressMe: HWND;
    

    match msg {
        winapi::winuser::WM_CREATE => {
            let hInstance = user32::GetWindowLongA(hwnd, GWL_HINSTANCE) as HINSTANCE;
            buttonDontPressMe = createHandle(0, "Button", "Don't press me",
                WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON, 100, 200, 50, 20,
                hwnd, Handle::Button, hInstance);
            buttonPressMe = createHandle(0, "Button", "Press me",
                WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON, 200, 200, 50, 20,
                hwnd, Handle::Button, hInstance);
        }
        winapi::winuser::WM_PAINT => {
            hdc = user32::BeginPaint(hwnd, lpPaintStruct);
            user32::GetClientRect(hwnd, lpRect);
            user32::DrawTextW(hdc, toWstring("Done with pride and prejudice by Culeva Alex"), -1, lpRect, DT_SINGLELINE | DT_CENTER | DT_VCENTER);
            user32::EndPaint(hwnd, lpPaintStruct);
        }
        winapi::winuser::WM_DESTROY => {
            user32::PostQuitMessage(0);
        }
        _ => {
                return user32::DefWindowProcW(hwnd, msg, wParam, lParam);
            }
    }

    return 0;
}

fn toWstring(str: &str) -> *const u16 {
    unsafe {
        let v: Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
        return v.as_ptr();
    }
}

fn hideConsoleWindow() {
    unsafe {
        let window = kernel32::GetConsoleWindow();
        if window != std::ptr::null_mut() {
            user32::ShowWindow(window, winapi::SW_HIDE);
        }
    }
}

fn main() {
    unsafe {
        hideConsoleWindow();

        let windowName = "PW_Laboratory_Work_1";
        let className = toWstring(windowName);
        let hInstance: HINSTANCE = 0 as HINSTANCE;

        let wnd = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW, lpfnWndProc: Some(windowProc), cbClsExtra: 0, cbWndExtra: 0,
            hInstance: hInstance, hIcon: user32::LoadIconW(hInstance, winapi::winuser::IDI_APPLICATION),
            hCursor: user32::LoadCursorW(hInstance, winapi::winuser::IDI_APPLICATION),
            hbrBackground: 16 as HBRUSH, lpszMenuName: 0 as LPCWSTR, lpszClassName: className,
        };

        user32::RegisterClassW(&wnd);
        let hwndDesktop = user32::GetDesktopWindow();

        createHandle(0, windowName, "PW_Laboratory_Work_1", WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                     0, 0, 400, 400, hwndDesktop, Handle::Window, hInstance);

        let mut msg = winapi::winuser::MSG {
            hwnd: 0 as HWND,
            message: 0 as UINT,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as DWORD,
            pt: winapi::windef::POINT { x: 0, y: 0 },
        };

        loop {
            let pm = user32::PeekMessageW(&mut msg, 0 as HWND, 0, 0, winapi::winuser::PM_REMOVE);

            if pm == 0 { continue; }

            if msg.message == winapi::winuser::WM_QUIT { break; }

            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }
    }
}

fn createHandle(dwExStyle: DWORD, lpClassName: &str, lpWindowName: &str, dwStyle: DWORD, x: c_int,
                y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, handle: Handle,
                hInstance: HINSTANCE) -> HWND {
    unsafe {
        return user32::CreateWindowExA(dwExStyle, lpClassName.as_ptr() as *mut _, lpWindowName.as_ptr() as *mut _, dwStyle,
                                       x, y, nWidth, nHeight, hWndParent, handle.value(), hInstance, std::ptr::null_mut())
    }
}
