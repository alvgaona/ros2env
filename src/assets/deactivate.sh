# Deactivate ROS 2 environment
export PATH=$(echo $PATH | tr ':' '\n' | grep -v '/opt/ros/' | tr '\n' ':')
unset ROS_DISTRO
unset ROS_VERSION
unset ROS_PYTHON_VERSION
unset AMENT_PREFIX_PATH
unset CMAKE_PREFIX_PATH
unset COLCON_PREFIX_PATH
unset PYTHONPATH
unset PKG_CONFIG_PATH
