module MarkdownParsing exposing (..)

import Html exposing (Html)
import Html.Attributes as Attr exposing (class, id)
import Markdown.Block as Block exposing (Block)
import Markdown.Parser exposing (deadEndToString)
import Markdown.Renderer exposing (Renderer, defaultHtmlRenderer)
import Regex
import Browser

main =
    Browser.element
        { init = \flags -> (flags, Cmd.none)
        , update = \_ model -> (model, Cmd.none)
        , subscriptions = \_ -> Sub.none
        , view = view
        }

type alias Model = String

view : Model -> Html msg
view model =
    Html.div [] (renderMarkdown customRenderer model)

renderMarkdown : Renderer (Html msg) -> String -> List (Html msg)
renderMarkdown renderer =
    Markdown.Parser.parse >>
    Result.mapError (List.map deadEndToString >> String.join "\n") >>
    Result.andThen (Markdown.Renderer.render renderer) >>
    Result.withDefault []

{-|
    Custom Markdown renderer, with all the Semantic UI stuff, etc.
 -}
customRenderer : Renderer (Html msg)
customRenderer =
    { defaultHtmlRenderer |
        heading = \{ level, rawText } ->
            case level of
                Block.H1 ->
                    Html.h1 [ class "ui dividing header", id <| makeIdFromText rawText] [ Html.text rawText ]

                Block.H2 ->
                    Html.h2 [ class "ui dividing header", id <| makeIdFromText rawText ] [ Html.text rawText ]

                Block.H3 ->
                    Html.h3 [ class "ui header", id <| makeIdFromText rawText ] [ Html.text rawText ]

                Block.H4 ->
                    Html.h4 [ class "ui header", id <| makeIdFromText rawText ] [ Html.text rawText ]

                Block.H5 ->
                    Html.h5 [ class "ui header", id <| makeIdFromText rawText ] [ Html.text rawText ]

                Block.H6 ->
                    Html.h6 [ class "ui header", id <| makeIdFromText rawText ] [ Html.text rawText ]
    , table = Html.table [ class "ui celled striped table" ]
    , unorderedList =
        Html.div [ class "ui bulleted list" ] <<
            List.map
                (\item ->
                    case item of
                        Block.ListItem task children ->
                            let
                                checkbox =
                                    case task of
                                        Block.NoTask -> Html.text ""
                                        Block.IncompleteTask ->
                                            Html.input
                                                [ Attr.disabled True
                                                , Attr.checked False
                                                , Attr.type_ "checkbox"
                                                ]
                                                []
                                        Block.CompletedTask ->
                                            Html.input
                                                [ Attr.disabled True
                                                , Attr.checked True
                                                , Attr.type_ "checkbox"
                                                ]
                                                []
                            in
                            Html.div [ class "item" ] (checkbox :: children)
                )
    , orderedList =
        \_ ->
            Html.ol [ class "ui ordered list" ] <<
                List.map (Html.li [ class "ui item" ])
    , thematicBreak = Html.div [ class "ui divider" ] []
    , codeBlock =
        \{ body, language } ->
            Html.pre [ class <| String.join " " ["hljs", Maybe.withDefault "text" language] ]
                [ Html.code [] [ Html.text body ] ]
    }

{-|
    A (somewhat hacky) way of assigning an id to a header based on its text content
    This is definitely something I'll have to come back to a few times
-}
makeIdFromText : String -> String
makeIdFromText =
    let weirdChars = Regex.fromString "[^A-Za-z0-9\\-_]+" in
    String.toLower >>
    String.replace "'" "" >>
    Regex.replace (Maybe.withDefault Regex.never weirdChars) (\m -> "-")