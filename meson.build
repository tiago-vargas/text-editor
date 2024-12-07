project(
	'text-editor',
	'rust',
	version: '0.1.0',
	meson_version: '>= 1.0.0',
	default_options: ['warning_level=2', 'werror=false'],
)

i18n = import('i18n')
gnome = import('gnome')

application_id = 'io.github.tiago_vargas.text_editor'
resource_prefix = '/io/github/tiago_vargas/text_editor'

subdir('data')
subdir('src')

gnome.post_install(
	glib_compile_schemas: true,
	gtk_update_icon_cache: true,
	update_desktop_database: true,
)
