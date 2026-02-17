_rosenv_strip() {
  echo "$1" | tr ':' '\n' | grep -v "/opt/ros/" | tr '\n' ':' | sed 's/:$//'
}

_rosenv_append() {
  local var_name="$1" dir="$2"
  if [ -d "$dir" ]; then
    local current
    eval "current=\$$var_name"
    if [[ ":${current}:" != *":${dir}:"* ]]; then
      eval "export $var_name=\"${current:+${current}:}${dir}\""
    fi
  fi
}
