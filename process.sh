dname=$(dirname "$0")
jack_comp_path=""
file="Main"
rm ${dname}/${file}.jack
cpp ${dname}/${file}.rs | sed s/'\\'/'\n'/g | sed s/^#[^$]*$// > ${dname}/${file}.jack

file="Cube"
rm ${dname}/${file}.jack
cpp ${dname}/${file}.rs | sed s/'\\'/'\n'/g | sed s/^#[^$]*$// > ${dname}/${file}.jack

file="Rooms"
rm ${dname}/${file}.jack
cpp ${dname}/${file}.rs | sed s/'\\'/'\n'/g | sed s/^#[^$]*$// > ${dname}/${file}.jack

bash ${jack_comp_path} ${dname}
