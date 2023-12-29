desktop_file_validate = find_program('desktop-file-validate')
if desktop_file_validate.found()
	test(
		'Validate desktop file',
		desktop_file_validate,
		args: ['com.github.tiago_vargas.text_editor.desktop']
	)
endif

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
	test(
		'Validate schema file',
		compile_schemas,
		args: ['--strict', '--dry-run', meson.current_source_dir()]
	)
endif
