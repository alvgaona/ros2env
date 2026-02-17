# Strip inherited /opt/ros paths from parent shell
export PATH=$(_rosenv_strip "$PATH")
export PYTHONPATH=$(_rosenv_strip "$PYTHONPATH")
export PKG_CONFIG_PATH=$(_rosenv_strip "$PKG_CONFIG_PATH")
export CMAKE_PREFIX_PATH=$(_rosenv_strip "$CMAKE_PREFIX_PATH")
export AMENT_PREFIX_PATH=$(_rosenv_strip "$AMENT_PREFIX_PATH")

export ROS_DISTRO="{distro}"
