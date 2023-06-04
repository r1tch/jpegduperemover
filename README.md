# JPEG Duplicate Remover


## Installation

...


## USAGE

  jpegduperemover <backupdir> <duplicates>

  jpegduperemover will recursively search for jpg images in `backupdir` and
  in `duplicates`, removing all duplicates from `duplicates`, leaving only images
  that were not found in `backupdir`.

  It uses file size and EXIF date information to determine duplicates (ie, no md5sum, etc)

  Purpose: backupdir is your permanent storage of images, the duplicates is the source (eg your phone).
  This way you can make sure that everything was really backed up, only requiring to review the ones not found in backupdir.
