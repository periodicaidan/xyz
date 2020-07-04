module TypedTitle exposing (main)

import Browser
import Html exposing (..)
import Time

main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

type alias Model =
    { text: String
    , shown: String
    , cursorVisible: Bool
    }

nextChar : Model -> Model
nextChar model =
    case String.uncons model.text of
        Just (c, rest) ->
            Model rest (model.shown ++ String.fromChar c) model.cursorVisible
        Nothing ->
            model

type alias Flags = String

init : Flags -> (Model, Cmd Msg)
init text =
    ( Model text "" True
    , Cmd.none
    )

type Msg
    = NextChar Time.Posix
    | CursorToggle Time.Posix

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        NextChar _ ->
            ( nextChar model, Cmd.none )
        CursorToggle _ ->
            ( { model | cursorVisible = not model.cursorVisible }, Cmd.none )

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch [
        if not (String.isEmpty model.text) then
                Time.every 100 NextChar
            else
                Sub.none
    , Time.every 500 CursorToggle
    ]


view : Model -> Html Msg
view model =
    text (model.shown ++ if model.cursorVisible then "█" else " ")