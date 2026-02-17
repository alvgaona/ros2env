# Append global ROS {distro} paths
_rosenv_append AMENT_PREFIX_PATH "{ros_root}"
_rosenv_append CMAKE_PREFIX_PATH "{ros_root}"
_rosenv_append PATH "{ros_root}/bin"
_rosenv_append PKG_CONFIG_PATH "{ros_root}/lib/pkgconfig"
for _rosenv_pypath in "{ros_root}"/lib/python*/site-packages; do
  _rosenv_append PYTHONPATH "$_rosenv_pypath"
done
unset _rosenv_pypath
