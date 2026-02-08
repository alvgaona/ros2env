#!/bin/zsh

echo "=== Testing rosenv ==="
echo ""

echo "1. Initial status:"
rosenv status
echo ""

echo "2. Activating Humble:"
eval $(rosenv activate humble)
echo "   ROS_DISTRO=$ROS_DISTRO"
echo "   AMENT_PREFIX_PATH=$AMENT_PREFIX_PATH"
echo ""

echo "3. Status after activation:"
rosenv status
echo ""

echo "4. Switching to Jazzy:"
eval $(rosenv activate jazzy)
echo "   ROS_DISTRO=$ROS_DISTRO"
echo "   AMENT_PREFIX_PATH=$AMENT_PREFIX_PATH"
echo ""

echo "5. Final status:"
rosenv status
echo ""

echo "6. Testing ros2 command:"
which ros2
ros2 pkg list | head -3
