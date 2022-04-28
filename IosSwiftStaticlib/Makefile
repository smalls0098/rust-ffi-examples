.PHONY: default
default:
	echo "USAGE: make lint/push"

.PHONY: lint
lint:
	pod lib lint IosSwiftStaticlib.podspec --private --allow-warnings --use-libraries --skip-import-validation --no-clean --verbose

.PHONY: push
push:
	pod spec lint IosSwiftStaticlib.podspec --allow-warnings
	pod trunk push IosSwiftStaticlib.podspec --allow-warnings --allow-warnings
