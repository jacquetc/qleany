// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_controller_export.h"
#include "undo_redo/undo_redo_scopes.h"
#include <QObject>

namespace Simple::Controller
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT UndoRedoSignals : public QObject
{
    Q_OBJECT
  public:
    explicit UndoRedoSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  Q_SIGNALS:

    /*!
     * \brief A signal that is emitted when the undo redo system state has
     *changed. Useful for the undo and redo
     * actions.
     */
    void stateChanged();

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     *start redoing.
     * actions.
     */
    void redoing(Simple::Controller::UndoRedo::Scope scope, bool active);

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     *start undoing.
     * actions.
     */
    void undoing(Simple::Controller::UndoRedo::Scope scope, bool active);
};
} // namespace Simple::Controller