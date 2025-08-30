import ts from 'typescript'
import MagicString from 'magic-string'

/**
 * @returns {{ script: import('svelte/compiler').Preprocessor }}
 */
function script() {
	return {
		async script({ attributes, content, filename = '' }) {
			if (
				attributes.lang !== 'ts' ||
				attributes.context === 'module' ||
				filename.includes('/node_modules/')
			) {
				return
			}

			if (filename !== '/Users/k/git/kadium/src/lib/Button.svelte') {
				return
			}
			console.log(`\n\n\n\n ${filename} :::::::::::::::::::`, attributes)
			console.log(content)
			console.log('\n ----------->\n')

			const source_file = ts.createSourceFile(
				filename || 'temp.ts',
				content,
				ts.ScriptTarget.Latest,
				true,
				ts.ScriptKind.TSX,
			)

			/** @type { name: string; type?: string; initializer?: string }[] */
			const exports = []

			// collect all `export let` declarations
			source_file.forEachChild((node) => {
				if (ts.isVariableStatement(node)) {
					const isExport = node.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)
					if (!isExport) return

					node.declarationList.declarations.forEach((decl) => {
						if (!ts.isIdentifier(decl.name)) return

						const name = decl.name.text
						const type = decl.type ? decl.type.getText(source_file) : undefined
						const initializer = decl.initializer ? decl.initializer.getText(source_file) : undefined

						exports.push({ name, type, initializer })
					})
				}
			})

			// if no props, do nothing
			if (!exports.length) return

			const s = new MagicString(content)

			// remove original export let lines
			for (const node of source_file.statements) {
				if (ts.isVariableStatement(node)) {
					const isExport = node.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)
					if (isExport) {
						s.remove(node.pos, node.end)
					}
				}
			}

			// construct the new destructure
			let prop_content = ''
			for (const e of exports) {
				prop_content += `\t${e.name}${e.initializer ? ` = ${e.initializer}` : ''},\n`
			}
			let type_content = ''
			for (const e of exports) {
				type_content += `\t${e.name}${e.type ? `: ${e.type}` : ''},\n`
			}
			const destructure = `let {\n${prop_content}}:  {\n${type_content}} = $props();\n`

			// prepend to <script>
			s.prepend(destructure)

			console.log(s.toString())

			return { code: s.toString(), map: s.generateMap({ hires: true, source: filename }) }
		},
	}
}

/**
 * @returns {import('svelte/compiler').PreprocessorGroup}
 */
export function props_preprocess() {
	/** @type {import('svelte/compiler').PreprocessorGroup} */
	const preprocessor = { name: 'vite-preprocess', script: script().script }
	return preprocessor
}
