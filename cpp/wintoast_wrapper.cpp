/* wintoast_wrapper.cpp => defines methods for sending notifications to the desktop call c++
 * MIT License
 * 
 * Copyright (c) 2024 Frederick Valdez
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#include "wintoastlib.h"
#include <string>
#include <windows.h>
#include <iostream>
#include <vector>

using namespace WinToastLib;

enum Results {
    Success = 0,
    SystemNotSupported = 1,
    InitializationFailure = 2,
    ToastFailed = 3
};

class CustomHandler : public IWinToastHandler {
public:
    void toastActivated() const {
        std::wcout << L"The user clicked in this toast" << std::endl;
        exit(0);
    }

    void toastActivated(int actionIndex) const {
        std::wcout << L"The user clicked on action #" << actionIndex << std::endl;
        exit(16 + actionIndex);
    }

    void toastDismissed(WinToastDismissalReason state) const {
        switch (state) {
            case UserCanceled:
                std::wcout << L"The user dismissed this toast" << std::endl;
                exit(1);
                break;
            case TimedOut:
                std::wcout << L"The toast has timed out" << std::endl;
                exit(2);
                break;
            case ApplicationHidden:
                std::wcout << L"The application hid the toast using ToastNotifier.hide()" << std::endl;
                exit(3);
                break;
            default:
                std::wcout << L"Toast not activated" << std::endl;
                exit(4);
                break;
        }
    }

    void toastFailed() const {
        std::wcout << L"Error showing current toast" << std::endl;
        exit(5);
    }
};

extern "C" {
int show_notification(const wchar_t* actions_acs,
                      const wchar_t* appName, 
                      const wchar_t* appUserModelID, 
                      const wchar_t* text, 
                      const wchar_t* imagePath, 
                      const wchar_t* attribute, 
                      INT64 expiration) {

    if (!WinToast::isCompatible()) {
        std::wcerr << L"Error, your system in not supported!" << std::endl;
        return Results::SystemNotSupported;
    }

    std::wstring appNameStr(appName);
    std::wstring appUserModelIDStr(appUserModelID);
    std::wstring textStr(text);
    std::wstring imagePathStr(imagePath);
    std::wstring attributeStr(attribute);
    std::wstring actions_ac(actions_acs);
    std::vector<std::wstring> actions;

    if (actions_acs != NULL && wcslen(actions_acs) > 0) {
       actions.push_back(actions_ac);
    }

    WinToastTemplate::AudioOption audioOption = WinToastTemplate::AudioOption::Default;

    WinToast::instance()->setAppName(appName);
    WinToast::instance()->setAppUserModelId(appUserModelID);

    if (!WinToast::instance()->initialize()) {
        std::wcerr << L"Error, your system in not compatible!" << std::endl;
        return Results::InitializationFailure;
    }

    WinToastTemplate templ((imagePath && wcslen(imagePath) > 0) ? WinToastTemplate::ImageAndText02 : WinToastTemplate::Text02);
    templ.setTextField(text, WinToastTemplate::FirstLine);
    templ.setAudioOption(audioOption);
    templ.setAttributionText(attribute);
    templ.setImagePath(imagePath);

    for (auto const& action : actions) {
        templ.addAction(action);
    }

    if (expiration) {
        templ.setExpiration(expiration);
    }

    if (WinToast::instance()->showToast(templ, new CustomHandler()) < 0) {
        std::wcerr << L"Could not launch your toast notification!";
        return Results::ToastFailed;
    }

    Sleep(expiration ? (DWORD)expiration + 1000 : 15000);
    return Results::Success;
 }
}
/*int wmain(int argc, LPWSTR* argv) {
    show_notification(L"", L"Console", L"WinToast", L"Hello Word", L"", L"default", 0);
    exit(2);
}*/
