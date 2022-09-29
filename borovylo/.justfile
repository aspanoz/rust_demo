set dotenv-load := true

synth_label := 'Borovylo'
control_label := 'Pompedullo MIDI'
gui_label := 'Jupen'

port := '5000'
gui_port := '3000'

jack := `jack_control status | rg -v status`

version := `toml get main/Cargo.toml package.version`
name := `toml get main/Cargo.toml bin.name`

# - выбор just рецепта, подготовка данных с описаниями для fzf preview
_default:
	#!/usr/bin/env bash
	clear;
	for recept in $(just --justfile {{justfile()}} --unsorted --summary); do
		description=$(just --justfile {{justfile()}} --show $recept 2> /dev/null | head -n 1| sed 's/# - //');
		echo "${recept}	${description}";
	done |\
		fzf --height=40% --layout=reverse --delimiter='\t' \
		--with-nth=1 --preview='echo {2}' --preview-window 'top:1%' |\
		sed 's/\s.*//g' | xargs just --justfile {{justfile()}} --working-directory {{invocation_directory()}}


# - запустить csound-синт
@run-csound-standalone:
	clear
	@# запуск standalone {{synth_label}}
	@csound/borovylo


# опционально включаю jack, по умолчанию alsa
# --exec 'run --bin pompedullo {{ if jack == "started" { "-p gamepad --features gamepad/jack" } else { "" } }}' \
# - cargo: запустить dev-сервер Pompedullo MIDI
@run-borovylo-dev:
	clear
	RUST_LOG=info systemfd --no-pid -s http::{{port}} -- cargo watch \
		--clear --quiet \
		--exec 'run --bin pompedullo' \
		--watch main/src/ \
		--watch web-client/src/ \
		--watch graphql/src/ \
		--watch gamepad/src/ \
		--watch midi/src/ \
		--watch gamepad/remap/remap.sdb
	@# {{control_label}} прощается и уходит


# - вывести список команд
@help:
	@# управление {{synth_label}}, версия {{version}}
	just --justfile {{justfile()}} --working-directory {{invocation_directory()}} --list --unsorted --list-heading ''
	echo ""


# - cargo: собрать Borovylo
@borovylo-build:
	cargo r --bin pompedullo
	@# собран "{{synth_label}}" версии {{version}}


# - npm: запустить dev-сервер Rylo
@run-gui-dev:
	cd web-client/js/
	clear
	@# {{gui_label}} {{`cd web-client/js/; jq -r ".name" package.json`}} v{{`cd web-client/js/; jq -r ".version" package.json`}}
	npm start -- --port {{gui_port}}


# - npm: собрать web-gui Rylo
@gui-build:
	cd web-client/js/
	clear
	@# подготовка gui {{gui_label}}
	rm -fr build
	npm run build 2>/dev/null
	@# собрано {{gui_label}} версии {{`cd web-client/js/; jq -r ".version" package.json`}}

# - cargo: генерация настроек ищ файла settings.toml
@settings:
	cargo settings all
	@# файл настроек settings.toml обработан


# - запустить в CabbageLite csound-синт, необходим jack, a2j
@run-csound-CabbageLite:
	clear
	@# запуск {{synth_label}} в CabbageLite
	CabbageLite csound/borovylo.csd


# - cargo: генерация gamepad файла настроек
# @remap-gamepad:
# 	cargo xtask sdb
# 	@# собран файл настроек gamepad для {{control_label}} версии {{version}}


# - cargo dev server debug, port=5000
# @debug:
# 	RUST_LOG=debug systemfd --no-pid -s http::5000 -- cargo watch \
# 		--exec 'run --bin borovylo' \
# 		--watch main/src/ \
# 		--watch web-client/src/ \
# 		--watch graphql/src/ \
# 		--watch gamepad/src/ \
# 		--watch gamepad/remap/remap.sdb \
# 		--watch .justfile
# - запустить собраное приложение
# @serve:
# 	cd web-client/js/
# 	serve build
# - увеличить на 1 version в package.json
# @version:
# 	echo "`jq '.version=\"{{package_next}}\"' package.json`" > package.json
# 	cd web-client/js/
# 	@# {{package_label}} изменение версии приложения, version = {{package_next}}
# - обновить зависимости package.json
# @update:
# 	cd web-client/js/
# 	ncu -u
# 	npm install
# 	npm audit fix
# 	@# {{package_label}} {{`jq -r ".version" package.json`}} обновление package.json
