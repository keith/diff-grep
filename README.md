# diff-grep

`diff-grep` is a tool for filtering diffs to only include the hunks
matching the given patterns.

## Example use

```sh
$ git diff ... > /tmp/original.diff

$ cat /tmp/original.diff
diff --git c/docs/ld.1.html w/docs/ld.1.html
index 2ac7626c..eeb7bdc5 100644
--- c/docs/ld.1.html
+++ w/docs/ld.1.html
@@ -897,7 +903,16 @@ When linking for two-level namespace, <code class="Nm">ld</code> does not verify
       build time. This option relaxes that requirement, allowing you to mix
       object files compiled for different ARM subtypes.</dd>
   <dt><a class="permalink" href="#no_uuid"><code class="Fl" id="no_uuid">-no_uuid</code></a></dt>
-  <dd>Do not generate an LC_UUID load command in the output file.</dd>
+  <dd>Do not generate an LC_UUID load command in the output file. Be warned that
+      binaries without UUIDs may cause the debugger and crash reporting tools to
+      be unable to track and inspect the binary.</dd>
+  <dt><a class="permalink" href="#random_uuid"><code class="Fl" id="random_uuid">-random_uuid</code></a></dt>
+  <dd>Generate a random LC_UUID load command in the output file. By default the
+      linker generates the UUID of the output file based on a hash of the output
+      file's content. But for very large output files, the hash can slow down
+      the link. Using a hash based UUID is important for reproducible builds,
+      but if you are just doing rapid debug builds, using -random_uuid may
+      improve turn around time.</dd>
   <dt><a class="permalink" href="#root_safe"><code class="Fl" id="root_safe">-root_safe</code></a></dt>
   <dd>Sets the MH_ROOT_SAFE bit in the mach header of the output file.</dd>
   <dt><a class="permalink" href="#setuid_safe"><code class="Fl" id="setuid_safe">-setuid_safe</code></a></dt>
@@ -1315,7 +1330,7 @@ as(1), ar(1), cc(1), nm(1), otool(1) lipo(1), arch(3), dyld(3), Mach-O(5),
 </div>
 <table class="foot">
   <tr>
-    <td class="foot-date">March 7, 2018</td>
+    <td class="foot-date">August 7, 2020</td>
     <td class="foot-os">Darwin</td>
   </tr>
 </table>

$ cat /tmp/original.diff | diff-grep foot-date --output /tmp/filtered.diff

$ cat /tmp/filtered.diff
--- c/docs/ld.1.html
+++ w/docs/ld.1.html
@@ -1315,7 +1330,7 @@ as(1), ar(1), cc(1), nm(1), otool(1) lipo(1), arch(3), dyld(3), Mach-O(5),
 </div>
 <table class="foot">
   <tr>
-    <td class="foot-date">March 7, 2018</td>
+    <td class="foot-date">August 7, 2020</td>
     <td class="foot-os">Darwin</td>
   </tr>
 </table>

$ git apply --reverse /tmp/filtered.diff
```

In this example we filter out noise from a diff, and the revert only
that hunk using `git apply --reverse`. This can be useful when you want
to stage, or checkout, large mechanical diffs that you have intertwined
other changes in.

## Installation

```sh
cargo install --path .
```
