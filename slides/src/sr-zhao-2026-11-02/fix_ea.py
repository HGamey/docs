import sys, zipfile, shutil, os
src, font = sys.argv[1], sys.argv[2]
tmp = src + ".tmp"
with zipfile.ZipFile(src) as zi, zipfile.ZipFile(tmp, "w", zipfile.ZIP_DEFLATED) as zo:
    for item in zi.infolist():
        data = zi.read(item.filename)
        if item.filename.startswith("ppt/theme/") and item.filename.endswith(".xml"):
            data = data.replace(b'<a:ea typeface=""/>', f'<a:ea typeface="{font}"/>'.encode())
        zo.writestr(item, data)
shutil.move(tmp, src)
