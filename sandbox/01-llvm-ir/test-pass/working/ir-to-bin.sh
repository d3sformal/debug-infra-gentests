cd ../hooklib
cmake ./
make

cd ../working/build
cp ../../hooklib/libmy-hook.so ./
$1 -fPIC ./bitcode.ll ./libmy-hook.so
cd ../