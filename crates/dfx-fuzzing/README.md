
>
> ``` sh
> echo core >/proc/sys/kernel/core_pattern
> ```

Warning one:
> Whoops, your system uses on-demand CPU frequency scaling, adjusted
> between 2148 and 3515 MHz. Unfortunately, the scaling algorithm in the
> kernel is imperfect and can miss the short-lived processes spawned by
> afl-fuzz. To keep things moving, run these commands as root:
> ``` sh
> cd /sys/devices/system/cpu
> echo performance | tee cpu*/cpufreq/scaling_governor
> ```
> You can later go back to the original state by replacing 'performance'with 'ondemand' or 'powersave'. If you don't want to change the settings,set AFL_SKIP_CPUFREQ to make afl-fuzz skip this check - but expect someperformance drop.
