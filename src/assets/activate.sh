# Strip inherited /opt/ros paths from parent shell
export PATH=$(_rosenv_strip "$PATH")
export PYTHONPATH=$(_rosenv_strip "$PYTHONPATH")
export PKG_CONFIG_PATH=$(_rosenv_strip "$PKG_CONFIG_PATH")
export CMAKE_PREFIX_PATH=$(_rosenv_strip "$CMAKE_PREFIX_PATH")
export AMENT_PREFIX_PATH=$(_rosenv_strip "$AMENT_PREFIX_PATH")

# Activate ROS 2 {distro}
export ROS_DISTRO="{distro}"
export ROS_VERSION="2"
export ROS_PYTHON_VERSION="3"

_rosenv_append AMENT_PREFIX_PATH "{ros_root}"
_rosenv_append CMAKE_PREFIX_PATH "{ros_root}"
_rosenv_append PATH "{ros_root}/bin"
_rosenv_append PKG_CONFIG_PATH "{ros_root}/lib/pkgconfig"
for _rosenv_pypath in "{ros_root}"/lib/python*/site-packages; do
  _rosenv_append PYTHONPATH "$_rosenv_pypath"
done
unset _rosenv_pypath

unset -f _rosenv_strip _rosenv_append
