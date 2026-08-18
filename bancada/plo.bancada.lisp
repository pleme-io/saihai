;; ─────────────────────────────────────────────────────────────────────
;; plo — the declared desktop.
;;
;; What the machine should look like. The loop reads this, reads the world,
;; and closes the difference; it never stops trying, and it plans nothing when
;; there is nothing to close.
;;
;; Every key this lowers to is addressed the way the action catalog addresses
;; what it OBSERVES, which is what makes a declaration routable. A key no
;; action observes surfaces as Gap::NoAction — visible, rather than a silent
;; no-op that reads as converged forever.
;;
;; There is deliberately no syntax here for a blind action. You cannot declare
;; a screenshot, because a screenshot is not a state a desktop can be held at:
;; firing it twice makes two files. The declarative surface admits exactly
;; what the reconciler can converge on.
;; ─────────────────────────────────────────────────────────────────────

(defbancada
  :node "plo"

  ;; Nord is the standardized fleet theme. pente renders it; saihai selects it.
  :theme (defwanttheme :name "nord")

  ;; plo's single output. The mode is left undeclared until the DRM handoff
  ;; lands — declaring a mode on a node whose only video device is still
  ;; simpledrm would be declaring intent the machine cannot yet reach, and the
  ;; loop would report an honest, permanent gap. Scale is safe to state now.
  :outputs ((defwantoutput :name "DP-1" :enabled #t :scale 1))

  ;; Keyboard control, declared as STATE rather than as events. Repeat rate and
  ;; delay are what a development desktop is actually judged on, and they are
  ;; readable, so the loop can hold them.
  :input (defwantinput
           :repeat-rate  50
           :repeat-delay 300
           :layout       "us"
           :mode         "default")

  ;; A session that should be unlocked. Stated rather than assumed: if
  ;; something locks it out from under the operator, the loop notices.
  :session (defwantsession :locked #f)

  ;; Bindings are state — a chord either maps to an action or it does not, and
  ;; that is readable, so drift in a keymap is drift the loop closes.
  :bindings ((defwantbinding :chord "Mod+Return" :action "launch-spawn")
             (defwantbinding :chord "Mod+q"      :action "window-close")
             (defwantbinding :chord "Mod+f"      :action "window-set-fullscreen")
             (defwantbinding :chord "Mod+h"      :action "window-focus-direction")
             (defwantbinding :chord "Mod+l"      :action "window-focus-direction")
             (defwantbinding :chord "Mod+1"      :action "workspace-focus")
             (defwantbinding :chord "Mod+2"      :action "workspace-focus")
             (defwantbinding :chord "Mod+Escape" :action "session-lock")))
