// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "undo_redo_command.h"

#include "simple_example_controller_export.h"
#include <QObject>
#include <QQueue>
#include <QSharedPointer>

namespace Simple::Controller::UndoRedo
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT UndoRedoStack : public QObject
{
    Q_OBJECT
  public:
    explicit UndoRedoStack(QObject *parent = nullptr);
    explicit UndoRedoStack(QObject *parent, const QUuid &id);

    QQueue<QSharedPointer<UndoRedoCommand>> &queue();
    void setQueue(const QQueue<QSharedPointer<UndoRedoCommand>> &newQueue);

    QUuid id() const;
    void setId(const QUuid &newId);

    int currentIndex() const;
    void setCurrentIndex(int newCurrentIndex);
    void incrementCurrentIndex();
    void decrementCurrentIndex();

  Q_SIGNALS:

  private:
    QUuid m_id;
    QQueue<QSharedPointer<UndoRedoCommand>> m_queue;
    int m_currentIndex = -1;
};
} // namespace Simple::Controller::UndoRedo