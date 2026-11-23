# $\textrm{\huge{pomodoro}}$

$\texttt{\textcolor{Red}{Another one!?} \textcolor{Orange}{Yep, there are many pomodoro CLIs, but this one is mine.}}$

The scope is intentionally minimal, the purpose I have for this is to be a statusbar module.

### As of writing this, what I want from this application is:

 - To display a timer on stdout, and therefore any statusbar (whether that's waybar, tmux, whatever).
 - To support advancing timers by config file (eg. a config file that says work periods of 25m, then a break of 5m, yadda yadda).
   - I have no intention of these being passed as command line arguments.
 - (Maybe) A command to watch a timer and emit events on a socket, or as different exit code. Probably scope-creep, as I am currently managing this just fine with a bash script against `pomodoro status`'s output 😅

### What I don't want:

 - A daemon, or a `pomodoro start` that doesn't exit until the timer is finished.
 - A bunch of features that, pragmatically, are not needed.
 - To lose a lot of time from other sideprojects to this one; I'm doing this to take a low-stakes break from the others.
