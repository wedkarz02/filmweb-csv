/**
 * DISCLAIMER:
 *
 * Please be aware that running scripts from unknown or untrusted sources can be dangerous.
 * Malicious scripts can potentially steal sensitive information, including your cookies,
 * which may give others access to your authenticated sessions.
 *
 * This script is provided to help you copy the 'Cookie' header from Filmweb to
 * authenticate your session.
 *
 * However, you should:
 * - Always be cautious when running scripts that manipulate your cookies.
 * - Ensure that you fully trust the source of the script before executing it.
 *
 * For more information on why this is necessary and how this script works, please refer
 * to the `README.md` file included with this project.
 *
 * By proceeding, you acknowledge the importance of protecting your sensitive data
 * and are aware of the potential risks.
 */

(function () {
    const cookies = document.cookie;
    const textarea = document.createElement("textarea");
    textarea.value = cookies;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand("copy");
    document.body.removeChild(textarea);
})();
