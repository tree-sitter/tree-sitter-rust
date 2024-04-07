package tree_sitter_rust_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-rust"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_rust.Language())
	if language == nil {
		t.Errorf("Error loading Rust grammar")
	}
}
